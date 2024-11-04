export enum HttpStatus {
  Ok = 200,
  BadRequest = 400,
}

export interface CreateResponse {
  is_succesful: boolean;
  error_message: string;
}
