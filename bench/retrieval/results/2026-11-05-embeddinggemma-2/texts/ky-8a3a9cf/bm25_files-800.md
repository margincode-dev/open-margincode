### test/response-consumption.ts
```typescript
import test from 'ava';
import ky, {HTTPError} from '../source/index.js';
import {createHttpTestServer} from './helpers/create-http-test-server.js';

const binary = Buffer.from([0, 255, 128, 13, 10, 0, 195, 40]);

// Fetch's Body mixin consumes bytes without text decoding for these methods.
// https://fetch.spec.whatwg.org/#body-mixin
for (const method of ['arrayBuffer', 'blob', 'bytes'] as const) {
	const run = method === 'bytes' && typeof Response.prototype.bytes !== 'function' ? test.skip : test;
	run(`${method} preserves non-text bytes through download progress`, async t => {
		const server = await createHttpTestServer(t);
		server.get('/', (_request, response) => {
			response.set({'content-type': 'application/octet-stream', 'content-length': String(binary.length)}).end(binary);
		});
		const chunks: Uint8Array[] = [];
		let finalTransferred = 0;
		const result = await ky(server.url, {
			onDownloadProgress(progress, chunk) {
				chunks.push(chunk);
				finalTransferred = progress.transferredBytes;
			},
		})[method]();
		if (method === 'blob') {
			t.true(result instanceof Blob);
		} else if (method === 'arrayBuffer') {
			t.true(result instanceof ArrayBuffer);
		} else {
			t.true(result instanceof Uint8Array);
		}

		const bytes = result instanceof Blob ? new Uint8Array(await result.arrayBuffer()) : new Uint8Array(result);
		t.deepEqual(Buffer.from(bytes), binary);
		t.deepEqual(Buffer.concat(chunks), binary);
		t.is(finalTransferred, binary.length);
		if (result instanceof Blob) {
			t.is(result.type, 'application/octet-stream');
			t.is(result.size, binary.length);
		}
	});
}

test('formData parses URL-encoded repeated fields, plus signs, and UTF-8 through progress', async t => {
	const server = await createHttpTestServer(t);
	const text = 'tag=one&tag=two&message=caf%C3%A9+time&literal=%2B&empty=';
	server.get('/', (_request, response) => {
		response.set('content-type', 'application/x-www-form-urlencoded').end(text);
	});
	const chunks: Uint8Array[] = [];
	const form = await ky(server.url, {
		onDownloadProgress(_progress, chunk) {
			chunks.push(chunk);
		},
	}).formData();

	t.deepEqual(form.getAll('tag'), ['one', 'two']);
	t.is(form.get('message'), 'café time');
	t.is(form.get('literal'), '+');
	t.is(form.get('empty'), '');
	t.is(Buffer.concat(chunks).toString(), text);
});

test('formData parses multipart file bytes and repeated fields', async t => {
	const server = await createHttpTestServer(t);
	const form = new FormData();
	form.append('tag', 'one');
	form.append('tag', 'two');
	form.append('file', new Blob([binary], {type: 'application/octet-stream'}), 'sample.bin');
	const encoded = new Response(form);
	const body = Buffer.from(await encoded.arrayBuffer());
	server.get('/', (_request, response) => {
		response.set('content-type', encoded.headers.get('content-type')!).end(body);
	});
	const parsed = await ky(server.url).formData();
	const file = parsed.get('file') as File;

	t.deepEqual(parsed.getAll('tag'), ['one', 'two']);
	t.true(file instanceof File);
	t.is(file.name, 'sample.bin');
	t.is(file.type, 'application/octet-stream');
	t.deepEqual(Buffer.from(await file.arrayBuffer()), binary);
});

