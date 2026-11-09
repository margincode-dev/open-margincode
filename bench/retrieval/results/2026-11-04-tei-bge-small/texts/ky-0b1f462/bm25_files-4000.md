### test/stream.ts
```typescript
import test, {type ExecutionContext} from 'ava';
import ky, {isNetworkError, type Progress} from '../source/index.js';
import {createLargeBlob} from './helpers/create-large-file.js';
import {createHttpTestServer} from './helpers/create-http-test-server.js';
import {parseRawBody, parseJsonBody} from './helpers/parse-body.js';

test('extending with undefined progress callbacks disables inherited tracking', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		response.end(await parseRawBody(request));
	});

	const instance = ky.create({
		onUploadProgress() {
			throw new Error('Inherited onUploadProgress must not run');
		},
		onDownloadProgress() {
			throw new Error('Inherited onDownloadProgress must not run');
		},
	}).extend({onUploadProgress: undefined, onDownloadProgress: undefined});

	t.is(await instance.post(server.url, {body: 'payload'}).text(), 'payload');
});

test('empty request body completes upload progress', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		response.json({body: await parseRawBody(request)});
	});

	const progressEvents: Array<{progress: Progress; chunk: Uint8Array}> = [];
	const response = await ky
		.post(server.url, {
			body: '',
			onUploadProgress(progress, chunk) {
				progressEvents.push({progress, chunk});
			},
		})
		.json<{body: string}>();

	t.is(response.body, '');
	t.deepEqual(progressEvents, [{
		progress: {percent: 1, totalBytes: 0, transferredBytes: 0},
		chunk: new Uint8Array(),
	}]);
});

for (const retryLimit of [0, 1]) {
	test(`upload progress preserves referrer options with retry limit ${retryLimit}`, async t => {
		const referrer = 'https://example.com/source';
		const referrerPolicy = 'no-referrer';
		const requests: Array<{referrer: string; referrerPolicy: string}> = [];
		let completedUploads = 0;

		const result = await ky.post('https://example.com', {
			body: 'payload',
			referrer,
			referrerPolicy,
			retry: {limit: retryLimit, methods: ['post'], delay: () => 0},
			async fetch(request) {
				requests.push({referrer: request.referrer, referrerPolicy: request.referrerPolicy});
				t.is(await request.text(), 'payload');
				return new Response('ok', {status: requests.length <= retryLimit ? 500 : 200});
			},
			onUploadProgress(progress) {
				if (progress.percent === 1) {
					completedUploads++;
				}
			},
		}).text();

		t.is(result, 'ok');
		t.is(completedUploads, retryLimit + 1);
		t.deepEqual(requests, Array.from({length: retryLimit + 1}, () => ({referrer, referrerPolicy})));
	});
}

for (const [name, options] of [
	['keepalive', {keepalive: true}],
	['no-cors', {mode: 'no-cors'}],
] as const) {
	test(`upload progress does not prevent ${name} requests`, async t => {
		const server = await createHttpTestServer(t, {bodyParser: false});
		server.post('/', async (request, response) => {
			response.end(await parseRawBody(request));
		});

		let progressCalls = 0;
		const result = await ky.post(server.url, {
			...options,
			body: 'payload',
			onUploadProgress() {
				progressCalls++;
			},
		}).text();

		t.is(result, 'payload');
		t.is(progressCalls, 0);
	});
}

test('POST JSON with upload progress', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		response.json(await parseRawBody(request));
	});

	const json = {test: 'test'};
	const data: Progress[] = [];
	const chunks: string[] = [];
	const responseJson = await ky
		.post(server.url, {
			json,
			onUploadProgress(progress, chunk) {
				data.push(progress);
				chunks.push(new TextDecoder().decode(chunk));
			},
		})
		.json();

	// Check if we have at least two progress updates
	t.true(data.length > 0, 'Should have at least one progress update');
	t.deepEqual(
		chunks,
		[
			'{"test":"test"}',
		],
		'Should have chunks for all events',
	);

	// Check the first progress update
	t.true(
		data[0].percent >= 0 && data[0].percent <= 1,
		'First update should have progress between 0 and 100%',
	);
	t.true(
		data[0].transferredBytes >= 0,
		'First update should have non-negative transferred bytes',
	);

	// Check intermediate updates (if any)
	for (let i = 1; i < data.length - 1; i++) {
		t.true(
			data[i].percent >= data[i - 1].percent,
			`Update ${i} should have higher or equal percent than previous`,
		);
		t.true(
			data[i].transferredBytes >= data[i - 1].transferredBytes,
			`Update ${i} should have more or equal transferred bytes than previous`,
		);
	}

	// Check the last progress update
	const lastUpdate = data.at(-1);
	t.is(lastUpdate.percent, 1, 'Last update should have 100% progress');
	t.true(
		lastUpdate.totalBytes > 0,
		'Last update should have positive total bytes',
	);
	t.is(
		lastUpdate.transferredBytes,
		lastUpdate.totalBytes,
		'Last update should have transferred all bytes',
	);
});

test('multiple beforeRequest Request hooks do not duplicate upload progress events', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		response.json(await parseJsonBody<Record<string, unknown>>(request));
	});

	let completedUploadProgressEvents = 0;
	const payload = {test: 'test'};
	const responseJson = await ky
		.post(server.url, {
			json: payload,
			hooks: {
				beforeRequest: [
					({request}) => {
						const headers = new Headers(request.headers);
						headers.set('x-hook-1', 'hook-1');
						return new Request(request, {headers});
					},
					({request}) => {
						const headers = new Headers(request.headers);
						headers.set('x-hook-2', 'hook-2');
						return new Request(request, {headers});
					},
				],
			},
			onUploadProgress(progress) {
				if (progress.percent === 1) {
					completedUploadProgressEvents++;
				}
			},
		})
		.json<Record<string, unknown>>();

	t.deepEqual(responseJson, payload);
	t.is(completedUploadProgressEvents, 1);
});

test('multiple beforeRequest Request hooks do not duplicate upload progress events across retries', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	let requestCount = 0;
	server.post('/', async (request, response) => {
		requestCount++;
		if (requestCount === 1) {
			response.status(500).json({error: 'retry'});
			return;
		}

		response.json(await parseJsonBody<Record<string, unknown>>(request));
	});

	const completedUploadProgressEventsPerAttempt: number[] = [0];
	const payload = {test: 'retry'};
	const responseJson = await ky
		.post(server.url, {
			json: payload,
			retry: {
				limit: 1,
				methods: ['post'],
			},
			hooks: {
				beforeRequest: [
					({request}) => {
						const headers = new Headers(request.headers);
						headers.set('x-hook-1', 'hook-1');
						return new Request(request, {headers});
					},
					({request}) => {
						const headers = new Headers(request.headers);
						headers.set('x-hook-2', 'hook-2');
						return new Request(request, {headers});
					},
				],
				beforeRetry: [
					() => {
						completedUploadProgressEventsPerAttempt.push(0);
					},
				],
			},
			onUploadProgress(progress) {
				if (progress.percent !== 1) {
					return;
				}

				const currentAttemptIndex = completedUploadProgressEventsPerAttempt.length - 1;
				completedUploadProgressEventsPerAttempt[currentAttemptIndex]++;
			},
		})
		.json<Record<string, unknown>>();

	t.deepEqual(responseJson, payload);
	t.deepEqual(completedUploadProgressEventsPerAttempt, [1, 1]);
});

test('beforeRequest Request replacement preserves upload progress sizing', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		let totalBytes = 0;
		for await (const chunk of request) {
			totalBytes += chunk.length as number;
		}

		response.json({receivedBytes: totalBytes});
	});

	const largeBlob = createLargeBlob(10);
	const progressEvents: Progress[] = [];
	const response = await ky
		.post(server.url, {
			body: largeBlob,
			hooks: {
				beforeRequest: [
					({request}) => {
						const headers = new Headers(request.headers);
						headers.set('x-hook', '1');
						return new Request(request, {headers});
					},
				],
			},
			onUploadProgress(progress) {
				progressEvents.push(progress);
			},
		})
		.json<{receivedBytes: number}>();

	t.true(progressEvents.length >= 2, 'Should produce multiple progress events');
	const nonFinalProgressEvents = progressEvents.filter(progress => progress.percent < 1);
	t.true(nonFinalProgressEvents.length > 0, 'Should include non-final progress events');
	for (const progress of nonFinalProgressEvents) {
		const expectedPercent = progress.transferredBytes / largeBlob.size;
		t.true(progress.percent <= expectedPercent + 0.01, 'Intermediate progress should reflect known body size');
	}

	const finalProgress = progressEvents.at(-1);
	t.truthy(finalProgress);
	t.is(finalProgress!.percent, 1);
	t.true(finalProgress!.totalBytes > 1024 * 1024);
	t.is(finalProgress!.totalBytes, finalProgress!.transferredBytes);
	t.is(response.receivedBytes, finalProgress!.totalBytes);
});

test('beforeRequest Request replacement with new body preserves upload progress sizing', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		let receivedBytes = 0;
		for await (const chunk of request) {
			receivedBytes += chunk.length as number;
		}

		response.json({receivedBytes});
	});

	const payload = {initial: true};
	const replacementBody = createLargeBlob(10);
	const progressEvents: Progress[] = [];
	const response = await ky
		.post(server.url, {
			json: payload,
			hooks: {
				beforeRequest: [
					({request}) => new Request(request, {body: replacementBody}),
				],
			},
			onUploadProgress(progress) {
				progressEvents.push(progress);
			},
		})
		.json<{receivedBytes: number}>();

	t.true(progressEvents.length > 0, 'Should produce progress events');
	const expectedSize = replacementBody.size;
	const finalProgress = progressEvents.at(-1);
	t.truthy(finalProgress);
	t.is(finalProgress!.percent, 1);
	t.is(finalProgress!.totalBytes, expectedSize);
	t.is(finalProgress!.transferredBytes, expectedSize);
	t.is(response.receivedBytes, expectedSize);
});

test('missing content-length still uses original body size for upload progress', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		let receivedBytes = 0;
		for await (const chunk of request) {
			receivedBytes += chunk.length as number;
		}

		response.json({receivedBytes});
	});

	const body = createLargeBlob(10);
	const progressEvents: Progress[] = [];
	const response = await ky
		.post(server.url, {
			body,
			hooks: {
				beforeRequest: [
					({request}) => {
						request.headers.delete('content-length');
					},
				],
			},
			onUploadProgress(progress) {
				progressEvents.push(progress);
			},
		})
		.json<{receivedBytes: number}>();

	t.true(progressEvents.length >= 2, 'Should produce multiple progress events');
	const nonFinalProgressEvents = progressEvents.filter(progress => progress.percent < 1);
	t.true(nonFinalProgressEvents.length > 0, 'Should include non-final progress events');
	t.true(nonFinalProgressEvents[0].percent > 0, 'Non-final progress should use known total size');

	const finalProgress = progressEvents.at(-1);
	t.truthy(finalProgress);
	t.is(finalProgress!.percent, 1);
	t.is(finalProgress!.totalBytes, body.size);
	t.is(finalProgress!.transferredBytes, body.size);
	t.is(response.receivedBytes, body.size);
});

test('beforeRequest replacement with smaller body completes upload progress correctly', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		let receivedBytes = 0;
		for await (const chunk of request) {
			receivedBytes += chunk.length as number;
		}

		response.json({receivedBytes});
	});

	const originalBody = createLargeBlob(10);
	const replacementBody = 'small-body';
	const replacementBodySize = Buffer.byteLength(replacementBody);
	const progressEvents: Progress[] = [];
	const response = await ky
		.post(server.url, {
			body: originalBody,
			hooks: {
				beforeRequest: [
					({request}) => new Request(request, {body: replacementBody}),
				],
			},
			onUploadProgress(progress) {
				progressEvents.push(progress);
			},
		})
		.json<{receivedBytes: number}>();

	t.true(progressEvents.length > 0, 'Should produce progress events');
	const nonFinalProgressEvents = progressEvents.filter(progress => progress.percent < 1);
	for (const progress of nonFinalProgressEvents) {
		t.true(progress.percent < 1, 'Intermediate progress should stay below completion');
	}

	const finalProgress = progressEvents.at(-1);
	t.truthy(finalProgress);
	t.is(finalProgress!.percent, 1);
	t.is(finalProgress!.transferredBytes, replacementBodySize);
	t.is(response.receivedBytes, replacementBodySize);
});

test('beforeRequest replacement with larger body has monotonic upload progress', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		let receivedBytes = 0;
		for await (const chunk of request) {
			receivedBytes += chunk.length as number;
		}

		response.json({receivedBytes});
	});

	const replacementBody = createLargeBlob(10);
	const progressEvents: Progress[] = [];
	const response = await ky
		.post(server.url, {
			body: 'tiny',
			hooks: {
				beforeRequest: [
					({request}) => new Request(request, {body: replacementBody}),
				],
			},
			onUploadProgress(progress) {
				progressEvents.push(progress);
			},
		})
		.json<{receivedBytes: number}>();

	t.true(progressEvents.length > 0, 'Should produce progress events');
	for (let index = 1; index < progressEvents.length; index++) {
		t.true(progressEvents[index].transferredBytes >= progressEvents[index - 1].transferredBytes, 'Transferred bytes should be monotonic');
		t.true(progressEvents[index].percent >= progressEvents[index - 1].percent, 'Percent should be monotonic');
	}

	const finalProgress = progressEvents.at(-1);
	t.truthy(finalProgress);
	t.is(finalProgress!.percent, 1);
	t.is(finalProgress!.transferredBytes, replacementBody.size);
	t.is(response.receivedBytes, replacementBody.size);
});

test('beforeRequest body replacement followed by header hook keeps progress and headers', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		let receivedBytes = 0;
		for await (const chunk of request) {
			receivedBytes += chunk.length as number;
		}

		response.json({
			receivedBytes,
			header: request.headers['x-hook-2'],
		});
	});

	const replacementBody = 'hook-replacement-body';
	const replacementBodySize = Buffer.byteLength(replacementBody);
	const progressEvents: Progress[] = [];
	const response = await ky
		.post(server.url, {
			body: createLargeBlob(10),
			hooks: {
				beforeRequest: [
					({request}) => new Request(request, {body: replacementBody}),
					({request}) => {
						const headers = new Headers(request.headers);
						headers.set('x-hook-2', 'true');
						return new Request(request, {headers});
					},
				],
			},
			onUploadProgress(progress) {
				progressEvents.push(progress);
			},
		})
		.json<{receivedBytes: number; header: string | undefined}>();

	const finalProgress = progressEvents.at(-1);
	t.truthy(finalProgress);
	t.is(finalProgress!.percent, 1);
	t.is(finalProgress!.transferredBytes, replacementBodySize);
	t.is(response.receivedBytes, replacementBodySize);
	t.is(response.header, 'true');
});

test('retry with beforeRequest body changes tracks per-attempt upload progress', async t => {
	const server = await createHttpTestServer(t, {bodyParser: false});
	let requestCount = 0;
	server.post('/', async (_request, response) => {
		requestCount++;
		if (requestCount === 1) {
			response.status(500).json({error: 'retry'});
			return;
		}

		response.status(200).json({ok: true});
	});

	const firstAttemptBody = 'first-attempt';
	const secondAttemptBody = 'second-attempt-body';
