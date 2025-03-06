import type OAuthApplication from "./OAuthApplication";

export default class OAuthConnection {
    _id: string;
    application: OAuthApplication;
    userId: string;
    scope: string[];
    expiresIn: number;
    createdAt: string;

    constructor(_id: string, application: OAuthApplication, userId: string, scope: string[], expiresIn: number, createdAt: string) {
        this._id = _id;
        this.application = application;
        this.userId = userId;
        this.scope = scope;
        this.expiresIn = expiresIn;
        this.createdAt = createdAt;
    }
}