export default class Passkey {
    id: string;
    name: string;
    createdAt: any;

    constructor(id: string, name: string, createdAt: any) {
        this.id = id;
        this.name = name;
        this.createdAt = createdAt;
    }

    static getCreatedAt(passkey: Passkey): Date {
        // @ts-ignore
        return new Date(parseInt(passkey.createdAt.$date.$numberLong) ?? 0);
    }
}