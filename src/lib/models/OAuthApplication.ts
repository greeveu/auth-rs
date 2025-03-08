export default class OAuthApplication {
    _id: string;
    name: string;
    description: string | null;
    redirectUris: string[];
    owner: string;
    createdAt: string;

    constructor(_id: string, name: string, description: string | null, redirectUris: string[], owner: string, createdAt: string) {
        this._id = _id;
        this.name = name;
        this.description = description;
        this.redirectUris = redirectUris;
        this.owner = owner;
        this.createdAt = createdAt;
    }
}