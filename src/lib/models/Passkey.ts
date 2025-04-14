export default class Passkey {
    id: string;
    deviceType: string;
    createdAt: any;

    constructor(id: string, deviceType: string, createdAt: any) {
        this.id = id;
        this.deviceType = deviceType;
        this.createdAt = createdAt;
    }

    static getCreatedAt(passkey: Passkey): Date {
        // @ts-ignore
        return new Date(parseInt(passkey.createdAt.$date.$numberLong) ?? 0);
    }
}