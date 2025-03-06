export default class OAuthApplicationUpdates {
    public name: string | null;
    public description: string | null;
    public redirectUris: string[] | null;

    constructor({ name, description, redirectUris }: { name: string | null, description: string | null, redirectUris: string[] | null }) {
        this.name = name;
        this.description = description;
        this.redirectUris = redirectUris;
    }
}