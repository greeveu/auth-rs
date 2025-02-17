class Role {
    _id: string;
    name: string;
    system: boolean;
    createdAt: Date;

    constructor(_id: string, name: string, system: boolean, createdAt: Date) {
        this._id = _id;
        this.name = name;
        this.system = system;
        this.createdAt = createdAt;
    }
}