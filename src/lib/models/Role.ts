export default class Role {
    _id: string;
    name: string;
    system: boolean;
    createdAt: any;

    constructor(_id: string, name: string, system: boolean, createdAt: any) {
        this._id = _id;
        this.name = name;
        this.system = system;
        this.createdAt = createdAt;
    }

    static getCreatedAt(role: Role): Date {
        // @ts-ignore
        return new Date(parseInt(role.createdAt.$date.$numberLong) ?? 0);
    }
}