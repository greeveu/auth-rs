export default class RoleUpdates {
    public name: string | null;

    constructor({ name }: { name: string | null }) {
        this.name = name;
    }
}