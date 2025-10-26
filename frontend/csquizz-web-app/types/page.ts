export interface Page<T> {
  items: T[];
  total_items: number;
  total_pages: number;
}