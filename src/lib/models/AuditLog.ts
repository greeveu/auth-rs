class AuditLog {
    public _id: string;
    public entity_id: string;
    public entity_type: AuditLogEntityType;
    public action: AuditLogAction;
    public reason: string;
    public author_id: string;
    public old_values: Map<string, string>;
    public new_values: Map<string, string>;
    public created_at: string;

    constructor(_id: string, entity_id: string, entity_type: AuditLogEntityType, action: AuditLogAction, reason: string, author_id: string, old_values: Map<string, string>, new_values: Map<string, string>, created_at: string) {
        this._id = _id;
        this.entity_id = entity_id;
        this.entity_type = entity_type;
        this.action = action;
        this.reason = reason;
        this.author_id = author_id;
        this.old_values = old_values;
        this.new_values = new_values;
        this.created_at = created_at;
    }
}

enum AuditLogAction {
    Create,
    Update,
    Delete
}

enum AuditLogEntityType {
    User,
    Role,
    OAuthApplication,
    Unknown
}