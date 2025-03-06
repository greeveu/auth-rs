import AuthStateManager from "./auth";
import type OAuthApplication from "./models/OAuthApplication";
import type OAuthApplicationUpdates from "./models/OAuthApplicationUpdates";
import type OAuthConnection from "./models/OAuthConnection";
import type UserMinimal from "./models/User";
import type UserUpdates from "./models/UserUpdates";

class AuthRsApi {
    public static baseUrl = 'http://localhost:8000/api';//'https://oauth.timlohrer.de/api';
    private token: string | null = null;
    private currentMfaFlowId: string | null = null;

    constructor() {}

    setToken(token: string | null) {
        this.token = token;
    }

    async checkOnlineState(): Promise<boolean> {
        const response = await fetch(AuthRsApi.baseUrl);

        return response.ok;
    }

    async login(email: string, password: string) {
        const response = await fetch(`${AuthRsApi.baseUrl}/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                if (data.data?.mfaRequired) {
                    this.currentMfaFlowId = data.data.mfaFlowId;
                    return data.data;
                } else {
                    throw new Error(data.message);   
                }
            }
            new AuthStateManager().setToken(data.data.token);
            this.token = data.data.token;
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async mfa(code: string) {
        if (!this.currentMfaFlowId) {
            throw new Error('No MFA flow ID');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/auth/mfa`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ code, flowId: this.currentMfaFlowId }),
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            new AuthStateManager().setToken(data.data.token);
            this.token = data.data.token;
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async enableMfa(user: UserMinimal, password: string) {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users/${user._id}/mfa/totp/enable`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({ password }),
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            if (data.data?.mfaRequired) {
                this.currentMfaFlowId = data.data.mfaFlowId;
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async disableMfa(user: UserMinimal, code: string | null, password: string | null): Promise<UserMinimal> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users/${user._id}/mfa/totp/disable`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({ code, password }),
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getCurrentUser(): Promise<UserMinimal> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users/@me`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async updateUser(user: UserMinimal, updates: UserUpdates): Promise<UserMinimal> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users/${user._id}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.token}`,
            },
            body: JSON.stringify(updates),
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getAllRoles(): Promise<Role[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/roles`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getConnections(user: UserMinimal): Promise<OAuthConnection[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users/${user._id}/connections`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async disconnectConnection(connection: OAuthConnection): Promise<null> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/connections/${connection.application._id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }	

    async getOAuthApplication(application: OAuthApplication): Promise<OAuthApplication> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications/${application._id}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getOAuthApplications(): Promise<OAuthApplication[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async authorizeOAuthApplication(clientId: string, redirectUri: string, scope: string[]) {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth/authorize`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({
                clientId,
                redirectUri,
                scope,
            })
        });

        if (response.ok) {
            const data = await response.json();
            return data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async updateOAuthApplication(application: OAuthApplication, updates: OAuthApplicationUpdates): Promise<OAuthApplication> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications/${application._id}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify(updates)
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async deleteOAuthApplication(application: OAuthApplication): Promise<OAuthApplication> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications/${application._id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            if (data.status != 200) {
                throw new Error(data.message);
            }
            return data.data;
        } else {
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }
}

export default AuthRsApi;