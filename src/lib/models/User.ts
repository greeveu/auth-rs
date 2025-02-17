class UserMinimal {
    _id: string;
    email: string;
    firstName: string;
    lastName: string;
    roles: string[];
    disabled: boolean;
    createdAt: Date;

    constructor(_id: string, email: string, firstName: string, lastName: string, roles: string[], disabled: boolean, createdAt: Date) {
        this._id = _id;
        this.email = email;
        this.firstName = firstName;
        this.lastName = lastName;
        this.roles = roles;
        this.disabled = disabled;
        this.createdAt = createdAt;
    }
}