export default class DateUtils {
  static getTimeString(date: Date): string {
      return `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}:${date.getSeconds().toString().padStart(2, '0')}`;
  }
  
  static getDateString(date: Date): string {
      return `${(date.getMonth() + 1).toString().padStart(2, '0')}/${date.getDate().toString().padStart(2, '0')}/${date.getFullYear()}`;
  }
  
  static getFullDateString(date: Date): string {
    return `${this.getDateString(date)} ${this.getTimeString(date)}`;
  }
}