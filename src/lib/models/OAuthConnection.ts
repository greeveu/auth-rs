import type OAuthApplication from "./OAuthApplication";

export default class OAuthConnection {
    _id: string;
    application: OAuthApplication;
    userId: string;
    scope: string[];
    expiresIn: number;
    createdAt: any;

    constructor(_id: string, application: OAuthApplication, userId: string, scope: string[], expiresIn: number, createdAt: any) {
        this._id = _id;
        this.application = application;
        this.userId = userId;
        this.scope = scope;
        this.expiresIn = expiresIn;
        this.createdAt = createdAt;
    }

    static getCreatedAt(connection: OAuthConnection): Date {
        // @ts-ignore
        return new Date(parseInt(connection.createdAt.$date.$numberLong) ?? 0);
    }

    static getExpiresAt(connection: OAuthConnection): Date {
        return new Date((parseInt(connection.createdAt.$date.$numberLong) + connection.expiresIn) - Date.now());
    }
}