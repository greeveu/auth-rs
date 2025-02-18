class OAuthApplication {
    id: string;
    name: string;
    description: string;
    redirectUris: string[];
    owner: string;
    createdAt: string;

    constructor(id: string, name: string, description: string, redirectUris: string[], owner: string, createdAt: string) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.redirectUris = redirectUris;
        this.owner = owner;
        this.createdAt = createdAt;
    }
};

export default OAuthApplication;