class AuditLog {
    public _id: string;
    public entityId: string;
    public entityType: AuditLogEntityType;
    public action: AuditLogAction;
    public reason: string;
    public authorId: string;
    public oldValues: Map<string, string>;
    public newValues: Map<string, string>;
    public createdAt: any;

    constructor(_id: string, entityId: string, entityType: AuditLogEntityType, action: AuditLogAction, reason: string, authorId: string, oldValues: Map<string, string>, newValues: Map<string, string>, createdAt: any) {
        this._id = _id;
        this.entityId = entityId;
        this.entityType = entityType;
        this.action = action;
        this.reason = reason;
        this.authorId = authorId;
        this.oldValues = oldValues;
        this.newValues = newValues;
        this.createdAt = createdAt;
    }

    static getCreatedAt(auditLog: AuditLog): Date {
        // @ts-ignore
        return new Date(parseInt(auditLog.createdAt.$date.$numberLong) ?? 0);
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