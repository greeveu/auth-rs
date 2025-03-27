export default class Settings {
    _id: string;
    allowRegistration: boolean;
    allowOauthAppsForUsers: boolean;

    constructor(_id: string, allowRegistration: boolean, allowOauthAppsForUsers: boolean) {
        this._id = _id;
        this.allowRegistration = allowRegistration;
        this.allowOauthAppsForUsers = allowOauthAppsForUsers;
    }
}