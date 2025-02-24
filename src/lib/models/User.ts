export default class UserMinimal {
    _id: string;
    email: string;
    firstName: string;
    lastName: string;
    roles: string[];
    disabled: boolean;
    createdAt: string;

    constructor(_id: string, email: string, firstName: string, lastName: string, roles: string[], disabled: boolean, createdAt: string) {
        this._id = _id;
        this.email = email;
        this.firstName = firstName;
        this.lastName = lastName;
        this.roles = roles;
        this.disabled = disabled;
        this.createdAt = createdAt;
    }

    static isSystemAdmin(user: UserMinimal): boolean {
        return user._id == '00000000-0000-0000-0000-000000000000';
    }

    static DEFAULT_USER_ID = '00000000-0000-0000-0000-000000000001';
}