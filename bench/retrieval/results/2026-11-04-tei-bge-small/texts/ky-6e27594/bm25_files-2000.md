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
