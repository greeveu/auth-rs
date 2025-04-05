# Passkey Authentication Documentation

## Overview

Passkeys allow users to authenticate without passwords using FIDO2/WebAuthn standards. This document outlines the API routes necessary for implementing passkey authentication in the auth-rs system.

## Routes

### 1. Register Passkey

#### `POST /auth/passkeys/register/start`

Initiates the passkey registration process for a user.

**Request:**
```json
{
  "userId": "UUID"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Passkey registration initiated",
  "data": {
    "challenge": "Base64EncodedChallenge",
    "registrationId": "UUID",
    "publicKey": {
      "rp": {
        "name": "auth-rs",
        "id": "localhost"
      },
      "user": {
        "id": "Base64EncodedUserId",
        "name": "user@example.com",
        "displayName": "User's Display Name"
      },
      "challenge": "Base64EncodedChallenge",
      "pubKeyCredParams": [
        {
          "type": "public-key",
          "alg": -7
        }
      ],
      "timeout": 60000,
      "attestation": "direct",
      "authenticatorSelection": {
        "authenticatorAttachment": "platform",
        "requireResidentKey": true,
        "residentKey": "required",
        "userVerification": "preferred"
      }
    }
  }
}
```

#### `POST /auth/passkeys/register/conditional`

Initiates a passkey registration that doesn't require a user account first - useful for new user registration with passkeys.

**Request:**
```json
{
  "userName": "user@example.com",
  "displayName": "User's Display Name"
}
```

**Response:**
Similar to the standard registration flow, but with resident key parameters enforced.

#### `POST /auth/passkeys/register/finish`

Completes the passkey registration process.

**Request:**
```json
{
  "registrationId": "UUID",
  "credential": {
    "id": "CredentialId",
    "rawId": "Base64EncodedRawId",
    "type": "public-key",
    "response": {
      "attestationObject": "Base64EncodedAttestationObject",
      "clientDataJSON": "Base64EncodedClientDataJSON"
    }
  }
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Passkey registered successfully",
  "data": {
    "id": "UUID",
    "deviceType": "passkey",
    "createdAt": "2023-01-01T12:00:00Z",
    "backupEligible": true,
    "backupState": false,
    "transports": null
  }
}
```

### 2. Authenticate with Passkey

#### `POST /auth/passkeys/authenticate/start`

Initiates the passkey authentication process with a username.

**Request:**
```json
{
  "email": "user@example.com"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Authentication initiated",
  "data": {
    "challenge": "Base64EncodedChallenge",
    "authenticationId": "UUID",
    "publicKey": {
      "challenge": "Base64EncodedChallenge",
      "rpId": "localhost",
      "allowCredentials": [
        {
          "type": "public-key",
          "id": "Base64EncodedCredentialId"
        }
      ],
      "timeout": 60000,
      "userVerification": "preferred"
    }
  }
}
```

#### `POST /auth/passkeys/authenticate/conditional`

Initiates authentication without requiring a username first (for resident keys).

**Request:**
Empty body or minimal information - no username required.

**Response:**
```json
{
  "success": true,
  "message": "Conditional authentication initiated",
  "data": {
    "challenge": "Base64EncodedChallenge",
    "authenticationId": "UUID",
    "publicKey": {
      "challenge": "Base64EncodedChallenge",
      "rpId": "yourdomain.com",
      "timeout": 60000,
      "userVerification": "required"
    }
  }
}
```

Note the absence of `allowCredentials` - this enables the browser to automatically prompt for any available resident key.

#### `POST /auth/passkeys/authenticate/finish`

Completes the passkey authentication process.

**Request:**
```json
{
  "authenticationId": "UUID",
  "credential": {
    "id": "CredentialId",
    "rawId": "Base64EncodedRawId",
    "type": "public-key",
    "response": {
      "authenticatorData": "Base64EncodedAuthenticatorData",
      "clientDataJSON": "Base64EncodedClientDataJSON",
      "signature": "Base64EncodedSignature",
      "userHandle": "Base64EncodedUserHandle"
    }
  }
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Authentication successful",
  "data": {
    "user": {
      "id": "UUID",
      "email": "user@example.com",
      "firstName": "John",
      "lastName": "Doe",
      "role": "user",
      "createdAt": "2023-01-01T12:00:00Z",
      "passkey_count": 1
    },
    "token": "JWTToken"
  }
}
```

### 3. Manage Passkeys

#### `GET /users/<user_id>/passkeys`

Retrieves all registered passkeys for the specified user.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```json
{
  "status": 200,
  "message": "Passkeys retrieved successfully",
  "data": [
    {
      "id": "UUID",
      "deviceType": "passkey",
      "createdAt": "2023-01-01T12:00:00Z"
    }
  ]
}
```

#### `PATCH /users/<user_id>/passkeys/<passkey_id>`

Updates a passkey's metadata.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Request:**
```json
{
  "deviceType": "phone"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Passkey updated successfully",
  "data": {
    "id": "UUID",
    "deviceType": "phone",
    "createdAt": "2023-01-01T12:00:00Z"
  }
}
```

#### `DELETE /users/<user_id>/passkeys/<passkey_id>`

Deletes a passkey.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```json
{
  "status": 200,
  "message": "Passkey deleted successfully",
  "data": null
}
```

## Configuration

The current implementation uses these default values:
- RP ID: "localhost"
- RP Name: "auth-rs"
- RP Origin: "http://localhost"

Note: These values are planned to be moved to a configuration file in a future update.

## Security Considerations

### Authorization and Authentication

1. **User Verification**: All passkey operations require proper authorization:
   - Registration requires an authenticated user session
   - Users can only register/manage passkeys for their own account
   - Passkey operations are protected by JWT token authentication

2. **Counter Validation**: 
   - Each passkey maintains a counter to prevent replay attacks
   - Counter is updated after successful authentication
   - Counter validation is performed during authentication

3. **Cross-User Protection**:
   - API endpoints validate that users can only access their own passkeys
   - User ID verification is performed for all passkey management operations

### Error Handling

The API provides specific error responses for common scenarios:
- Invalid or expired registration/authentication sessions
- Missing or invalid credentials
- Unauthorized access attempts
- Non-existent passkeys or users

## Implementation Notes

1. **Passkey Model**:
```json
{
  "id": "UUID",
  "credentialId": "Base64EncodedString",
  "publicKey": "String",
  "counter": "Integer",
  "transports": ["String"] | null,
  "backupEligible": "Boolean",
  "backupState": "Boolean",
  "deviceType": "String",
  "createdAt": "DateTime"
}
```

2. **Response Format**: All API responses follow a consistent format:
```json
{
  "status": "Integer",
  "message": "String",
  "data": "T | null"
}
```

## Planned Features

The following features are planned for future implementation:

1. **Conditional Registration**: Allow passkey registration during initial user signup
2. **Conditional Authentication**: Support usernameless authentication with resident keys
3. **Configuration Management**: Move WebAuthn configuration to external config file

## Testing

1. Test across multiple browsers and platforms to ensure compatibility
2. Test with both platform authenticators (e.g., Windows Hello, Touch ID) and cross-platform authenticators (e.g., YubiKey)
3. Test registration, authentication, and management flows thoroughly
4. Test error scenarios and edge cases
5. Test authorization and security measures:
   - Cross-user access attempts
   - Counter replay attacks
   - Invalid token access

## Example Implementation Notes

1. **Storage Requirements**: Resident keys require more storage on authenticator devices. Some hardware authenticators have limited storage, while platform authenticators generally have no practical limit.

2. **Credential Discovery**: For usernameless authentication, no `allowCredentials` list is provided, allowing the browser to check for any resident credentials that match your Relying Party ID.

3. **WebAuthn Library**: The implementation will use the existing `webauthn-rs` library, which provides a comprehensive API for handling all passkey operations. 