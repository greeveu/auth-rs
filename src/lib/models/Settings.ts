export default class Settings {
    _id: string;
    openRegistration: boolean;
    allowOauthAppsForUsers: boolean;

    constructor(_id: string, openRegistration: boolean, allowOauthAppsForUsers: boolean) {
        this._id = _id;
        this.openRegistration = openRegistration;
        this.allowOauthAppsForUsers = allowOauthAppsForUsers;
    }
}