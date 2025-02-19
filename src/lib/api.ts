import AuthStateManager from "./auth";

class AuthRsApi {
    public static baseUrl = 'http://localhost:8000/api';//'https://oauth.timlohrer.de/api';//
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

    async getOAuthApplication(id: string): Promise<OAuthApplication> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications/${id}`, {
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
}

export default AuthRsApi;