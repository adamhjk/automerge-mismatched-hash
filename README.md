# Automerge mismatched hashes bug

This repository is a reproduction of a mismatched hash bug in automerge. To try it yourself:

## Setup

### Call setup.sh with your desired branches

For example, if you want to test the current `performance` branch of automerge with the `main` branch
of automerge-rs, you would call:

```
$ ./setup.sh performance main
```

The script will then install everything, along with the correct branches you want to test. It will
also build automerge (js).

### Start the server

To compile and start the rust server, run `npm start`

### Run the test

To run the tests: `npm test`

## Current results

```
> automerge-bad-hashes@1.0.0 test /home/adam/src/automerge-bad-hashes
> jest ./checksum.test.js

 FAIL  ./checksum.test.js
  ✓ good doc is transferred (31 ms)
  ✕ bad doc is transferred (135 ms)

  ● bad doc is transferred

    RangeError: Mismatched heads hashes: expected ba9188f5f02600127cfc6d47a4f57e6eacb10ea9827acec6daf75d6cd978f8dc, got cc8d4062def534c67a7dd9373b520ebf196f5516175a104dd7a1329542c0dee1

      at decodeDocumentChanges (automerge/dist/webpack:/Automerge/backend/columnar.js:1006:11)
      at decodeDocument (automerge/dist/webpack:/Automerge/backend/columnar.js:1068:3)
      at decodeChanges (automerge/dist/webpack:/Automerge/backend/columnar.js:754:34)
      at Object.backend [as load] (automerge/dist/webpack:/Automerge/backend/index.js:195:27)
      at Object.load (automerge/dist/webpack:/Automerge/src/automerge.js:51:17)
      at Object.<anonymous> (checksum.test.js:16:23)

Test Suites: 1 failed, 1 total
Tests:       1 failed, 1 passed, 2 total
Snapshots:   0 total
Time:        0.82 s, estimated 1 s
Ran all test suites matching /.\/checksum.test.js/i.
npm ERR! Test failed.  See above for more details.
```
