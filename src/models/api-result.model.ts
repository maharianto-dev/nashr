export interface BaseApiResult<T> {
  is_successful: boolean,
  message: string,
  payload: T | null
}