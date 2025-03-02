export default class UserUpdates {
    email: string | null;
    password: string | null;
    firstName: string | null;
    lastName: string | null;
    roles: string[] | null;
    disabled: boolean | null;

    constructor({ email, password, firstName, lastName, roles, disabled }: { email: string | null, password: string | null, firstName: string | null, lastName: string | null, roles: string[] | null, disabled: boolean | null }) {
        this.email = email;
        this.password = password;
        this.firstName = firstName;
        this.lastName = lastName;
        this.roles = roles;
        this.disabled = disabled;
    }
}