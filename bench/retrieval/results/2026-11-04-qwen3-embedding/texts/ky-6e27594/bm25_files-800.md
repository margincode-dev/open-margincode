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
