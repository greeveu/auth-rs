class UserMinimal {
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
}