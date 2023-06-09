/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** Writes a credential. */
export function writeCredential(target: string, username: string, secret: string): void
/** Reads a credential. Throws Error with code: 'InvalidArg' if the credential doesn't exist */
export function readCredential(target: string): NapiCredentials
/** Deletes a credential. Throws Error with code: 'InvalidArg' if the credential doesn't exist */
export function deleteCredential(target: string): void
export interface NapiCredentials {
  username: string
  secret: string
}
