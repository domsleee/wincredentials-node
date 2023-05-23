import test from 'ava'

import { deleteCredential, readCredential, writeCredential } from '../index.js'

test('reading and writing credentials', (t) => {
  writeCredential('wincredentials-node', 'myUsername', 'mySecret')
  t.deepEqual(readCredential('wincredentials-node'), { username: 'myUsername', secret: 'mySecret' })
  writeCredential('wincredentials-node', 'myUsername2', 'mySecret2')
  t.deepEqual(readCredential('wincredentials-node'), { username: 'myUsername2', secret: 'mySecret2' })
})

test('reading non-existent credential throws exception', (t) => {
  t.throws(
    () => {
      readCredential('wincredentials-node2')
    },
    { instanceOf: Error, code: 'InvalidArg', message: 'unable to read_credential: Element not found.' },
  )
})

test('deleting non-existent credential throws exception', (t) => {
  t.throws(
    () => {
      deleteCredential('wincredentials-node2')
    },
    { instanceOf: Error, code: 'InvalidArg', message: 'unable to delete_credential: Element not found.' },
  )
})
