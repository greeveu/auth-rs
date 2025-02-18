class OAuthApplication {
    id: string;
    name: string;
    description: string;
    redirectUris: string[];
    owner: string;
    createdAt: Date;

    constructor(id: string, name: string, description: string, redirectUris: string[], owner: string, createdAt: Date) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.redirectUris = redirectUris;
        this.owner = owner;
        this.createdAt = createdAt;
    }
};

export default OAuthApplication;