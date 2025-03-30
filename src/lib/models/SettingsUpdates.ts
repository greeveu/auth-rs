export default class SettingsUpdates {
    public openRegistration: boolean | null;
    public allowOauthAppsForUsers: boolean | null;

    constructor({ openRegistration, allowOauthAppsForUsers }: { openRegistration: boolean | null, allowOauthAppsForUsers: boolean | null }) {
        this.openRegistration = openRegistration;
        this.allowOauthAppsForUsers = allowOauthAppsForUsers;
    }
}