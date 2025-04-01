export default class RegistrationToken {
    _id: string;
    code: string;
    maxUses: number;
    uses: string[];
    autoRoles: string[];
    expiresIn: number | null;
    expiresFrom: any;
    createdAt: any;

    constructor(_id: string, code: string, maxUses: number, uses: string[], autoRoles: string[], expiresIn: number | null, expiresFrom: any, createdAt: any) {
        this._id = _id;
        this.code = code;
        this.maxUses = maxUses;
        this.uses = uses;
        this.autoRoles = autoRoles;
        this.expiresIn = expiresIn;
        this.expiresFrom = expiresFrom;
        this.createdAt = createdAt;
    }

    static getUrl(token: RegistrationToken): string {
        return `${document.location.origin}/register?registration_code=${token.code}`;
    }

    static getCreatedAt(token: RegistrationToken): Date {
        // @ts-ignore
        return new Date(parseInt(token.createdAt.$date.$numberLong) ?? 0);
    }

    static getExpiresAt(token: RegistrationToken): Date | null {
        return token.expiresIn ? new Date((parseInt(token.expiresFrom.$date.$numberLong) + token.expiresIn) - Date.now()) : null;
    }
}