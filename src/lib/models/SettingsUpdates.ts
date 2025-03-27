export default class SettingsUpdates {
    public allowRegistration: boolean;
    public allowOauthAppsForUsers: boolean;

    constructor({ allowRegistration, allowOauthAppsForUsers }: { allowRegistration: boolean, allowOauthAppsForUsers: boolean }) {
        this.allowRegistration = allowRegistration;
        this.allowOauthAppsForUsers = allowOauthAppsForUsers;
    }
}