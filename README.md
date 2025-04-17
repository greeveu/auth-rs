# Auth-RS

A Rust-based OAuth2 and webauthen compatible authentication system.

## Hosting Instructions

To self-host `auth-rs`, use the provided [`docker-compose.yml`](docker-compose.yml) file located in the root of this repository.

### Environment Variables

Before starting the services, ensure the following environment variables are properly configured in the `docker-compose.yml` file:

#### Backend Service (`auth-rs-backend`)

- **`ROCKET_ADDRESS`**: The address the backend server will bind to.  
  - **Default**: `0.0.0.0` (binds to all network interfaces).  
  - **Note**: Do not change this unless necessary.

- **`ROCKET_DATABASES`**: The database connection string for MongoDB.  
  - **Default**: `{auth-rs-db={url="mongodb://mongodb:27017"}}`.  
  - **Note**: Update the `url` if you modify the MongoDB service name, port, or credentials.

- **`SYSTEM_EMAIL`**: The email address for the system administrator.  
  - **Example**: `admin@example.com`.  
  - **Note**: Required for administrative tasks. Does not have to be an actual email, but has to look like one.

- **`SYSTEM_PASSWORD`**: A strong password for the system administrator.  
  - **Example**: `P@ssw0rd123!`. -> DO NOT USE THIS ONE!  
  - **Note**: Ensure this is secure to protect the admin account.

- **`WEBAUTHN_RP_ID`**: The domain name of the Relying Party (RP) for WebAuthn.  
  - **Example**: `yourdomain.com`.  
  - **Note**: Do not include `http://`, `https://`, or subdomains.

- **`WEBAUTHN_RP_ORIGIN`**: The full URL of your frontend application.  
  - **Example**: `https://subdomain.yourdomain.com`.  
  - **Note**: Must match the origin of your frontend for WebAuthn to work.

- **`WEBAUTHN_RP_NAME`**: The name of the Relying Party (RP) displayed to users during WebAuthn authentication.  
  - **Example**: `auth-rs`.  
  - **Note**: Avoid spaces or special characters.

- **`TOTP_ISSUER_NAME`**: The name of the TOTP (Time-based One-Time Password) issuer displayed to users in their authenticator app.  
  - **Example**: `auth-rs`.  
  - **Note**: Avoid spaces or special characters.

#### Frontend Service (`auth-rs-frontend`)

- **`PUBLIC_API_URL`**: The base URL for the backend API that the frontend will communicate with.  
  - **Example**: `https://yourdomain.com/api`.  
  - **Note**: Ensure this matches the actual URL where your backend is hosted.

---

### Starting the Services

Once the environment variables are configured, start the services using Docker Compose:
```bash
docker-compose up -d
```

The backend will be available at port 8000 and the frontend at port 3000.
I suggest using nginx custom-paths to map the backend into the domains /api path or using a subdomain to host you backend like api.exmaple.com.

## Legal

Icons: <https://lucide.dev/license>

MIT Licence
