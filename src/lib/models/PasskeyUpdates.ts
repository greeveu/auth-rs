export default class PasskeyUpdates {
    deviceType: string | null;

    constructor({ deviceType }: { deviceType: string | null }) {
        this.deviceType = deviceType;
    }
}