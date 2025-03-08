export default class OAuthApplication {
    _id: string;
    name: string;
    description: string | null;
    redirectUris: string[];
    owner: string;
    createdAt: any;

    constructor(_id: string, name: string, description: string | null, redirectUris: string[], owner: string, createdAt: any) {
        this._id = _id;
        this.name = name;
        this.description = description;
        this.redirectUris = redirectUris;
        this.owner = owner;
        this.createdAt = createdAt;
    }

    static getCreatedAt(application: OAuthApplication): Date {
        // @ts-ignore
        return new Date(parseInt(application.createdAt.$date.$numberLong) ?? 0);
    }
}