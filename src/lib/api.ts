import AuthStateManager from "./auth";
import type { AuditLog } from "./models/AuditLog";
import type OAuthApplication from "./models/OAuthApplication";
import type OAuthApplicationUpdates from "./models/OAuthApplicationUpdates";
import type OAuthConnection from "./models/OAuthConnection";
import type RegistrationToken from "./models/RegistrationToken";
import type RegistrationTokenUpdates from "./models/RegistrationTokenUpdates";
import type Role from "./models/Role";
import type RoleUpdates from "./models/RoleUpdates";
import type Settings from "./models/Settings";
import type SettingsUpdates from "./models/SettingsUpdates";
import type User from "./models/User";
import type UserUpdates from "./models/UserUpdates";

class AuthRsApi {
    public static baseUrl = 'http://localhost:8000/api';//'http://localhost:8000/api';
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

    async getSettings(): Promise<Settings> {
        const response = await fetch(`${AuthRsApi.baseUrl}/settings`, {
            method: 'GET'
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async updateSettings(updates: SettingsUpdates): Promise<Settings> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/admin/settings`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.token}`,
            },
            body: JSON.stringify(updates),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
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
            if (data.data?.mfaRequired) {
                this.currentMfaFlowId = data.data.mfaFlowId;

                return data.data;
            }
            new AuthStateManager().setToken(data.data.token);
            this.token = data.data.token;
            return data.data;
        } else {
            console.error((await response.json()));
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
            new AuthStateManager().setToken(data.data.token);
            this.token = data.data.token;
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async enableMfa(user: User, password: string) {
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
            if (data.data?.mfaRequired) {
                this.currentMfaFlowId = data.data.mfaFlowId;
            }
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async disableMfa(user: User, code: string | null, password: string | null): Promise<User> {
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
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async createUser(email: string, password: string, firstName: string, lastName: string, registrationCode: string | null): Promise<User> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({ email, password, firstName, lastName, registrationCode }),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getCurrentUser(): Promise<User> {
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
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getAllUsers(): Promise<User[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async updateUser(user: User, updates: UserUpdates): Promise<User> {
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
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async deleteUser(user: User): Promise<User> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users/${user._id}`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async createRole(name: string): Promise<Role> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/roles`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({ name }),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
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
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getRole(roleId: string): Promise<Role> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/roles/${roleId}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async updateRole(role: Role, updates: RoleUpdates): Promise<Role> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/roles/${role._id}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify(updates),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async deleteRole(role: Role): Promise<Role> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/roles/${role._id}`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getConnections(user: User): Promise<OAuthConnection[]> {
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
            return data.data;
        } else {
            console.error((await response.json()));
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
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async createOAuthApplication(name: string, description: string | null, redirectUris: string[]): Promise<OAuthApplication> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({
                name,
                description,
                redirectUris
            }),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }


    async getOAuthApplication(clientId: string): Promise<OAuthApplication> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/oauth-applications/${clientId}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
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
            return data.data;
        } else {
            console.error((await response.json()));
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
            console.error((await response.json()));
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
            return data.data;
        } else {
            console.error((await response.json()));
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
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getAuditLogs(user: User | null): Promise<AuditLog[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        let url: string;
        if (user) {
            url = `${AuthRsApi.baseUrl}/users/${user._id}/audit-logs`;
        } else {
            url = `${AuthRsApi.baseUrl}/audit-logs`;
        }

        const response = await fetch(url, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getUsers(): Promise<User[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/users`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async createRegistrationToken(maxUses: number, expiresIn: number | null): Promise<RegistrationToken> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/registration-tokens`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify({ maxUses, expiresIn }),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getRegistrationToken(tokenId: string): Promise<RegistrationToken> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/registration-tokens/${tokenId}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async getAllRegistrationTokens(): Promise<RegistrationToken[]> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/registration-tokens`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async updateRegistrationToken(token: RegistrationToken, updates: RegistrationTokenUpdates): Promise<RegistrationToken> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/registration-tokens/${token._id}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
            body: JSON.stringify(updates),
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }

    async deleteRegistrationToken(token: RegistrationToken): Promise<RegistrationToken> {
        if (!this.token) {
            throw new Error('No token');
        }

        const response = await fetch(`${AuthRsApi.baseUrl}/registration-tokens/${token._id}`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${this.token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            return data.data;
        } else {
            console.error((await response.json()));
            throw new Error(`(${response.status}): ${response.statusText}`);
        }
    }
}

export default AuthRsApi;