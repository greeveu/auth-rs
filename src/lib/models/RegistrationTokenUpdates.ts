export default class RegistrationTokenUpdates {
    public maxUses: number | null;
    public expiresIn: number | null;
    public autoRoles: string[] | null;

    constructor({ maxUses, expiresIn, autoRoles }: { maxUses: number | null; expiresIn: number | null; autoRoles: string[] | null }) {
        this.maxUses = maxUses;
        this.expiresIn = expiresIn;
        this.autoRoles = autoRoles;
    }
}