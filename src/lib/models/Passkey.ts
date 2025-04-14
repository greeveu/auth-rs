export default class Passkey {
    id: string;
    owner: string;
    name: string;
    createdAt: any;

    constructor(id: string, owner: string, name: string, createdAt: any) {
        this.id = id;
        this.owner = owner;
        this.name = name;
        this.createdAt = createdAt;
    }

    static getCreatedAt(passkey: Passkey): Date {
        // @ts-ignore
        return new Date(parseInt(passkey.createdAt.$date.$numberLong) ?? 0);
    }
}