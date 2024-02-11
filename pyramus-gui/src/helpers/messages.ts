import { type ClassConstructor, plainToInstance } from 'class-transformer'
import { subscribeFrontendCommand } from '/wasm/pkg/pyramus_wasm'

export class JsMessage {
  // The marker provides a way to check if an object is a sub-class constructor for a jsMessage.
  static readonly jsMessageMarker = true
}

export class Rerender extends JsMessage {}
export class UpdateStage extends JsMessage {}

// `any` is used since the type of the object should be known from the Rust side
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type JSMessageFactory = (data: any) => JsMessage
type MessageMaker = typeof JsMessage | JSMessageFactory

export const messageMakers: Record<string, MessageMaker> = {
  Rerender,
  UpdateStage,
} as const
export type JsMessageType = keyof typeof messageMakers

export function subscribe<T extends JsMessage>(
  messageType: JsMessageType,
  callback: (messageData: T) => void
) {
  const messageMaker: JsMessage = messageMakers[messageType]
  if (!messageMaker) {
    // eslint-disable-next-line no-console
    console.error(
      `Received a frontend message of type "${messageType}" but was not able to parse the data. ` +
        "(Perhaps this message parser isn't exported in `messageMakers` at the bottom of `messages.ts`.)"
    )
    return
  }

  const outerCallback = (messageType: JsMessageType, messageData: Record<string, unknown>) => {
    console.log('Received message', messageType, messageData)
    const msg: JsMessage = plainToInstance(
      messageMaker.constructor as ClassConstructor<JsMessage>,
      {}
    )
    callback(msg as T)
  }

  subscribeFrontendCommand(messageType, outerCallback)
}
