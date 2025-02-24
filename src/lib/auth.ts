import { goto } from "$app/navigation";
import AuthRsApi from "./api";
import type UserMinimal from "./models/User";

class AuthStateManager {
    constructor() {}

    getToken() {
        return localStorage.getItem('token');
    }

    setToken(token: string) {
        localStorage.setItem('token', token);
    }

    clearToken() {
        localStorage.removeItem('token');
    }

    async handlePageLoad(params: string[] | null = null): Promise<[AuthRsApi, UserMinimal] | null> {
        const token = this.getToken();
        if (token) {
            const api = new AuthRsApi();
            api.setToken(token);
            try {
                const user = await api.getCurrentUser();
                return [api, user];
            } catch {
                this.clearToken();
                goto(`/login${params ? `?${params.join('&')}` : ''}`);
                return null;
            }
        } else {
            goto(`/login${params ? `?${params.join('&')}` : ''}`);
            return null;
        }
    }

    logout() {
        this.clearToken();
        goto(`/logout`);
    }
}

export default AuthStateManager;