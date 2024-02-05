import { ofetch } from 'ofetch'

export const useFetch = async (url : RequestInfo, item : String, isSilent : boolean) => {
  try {
    // TODO: Dynamic version
    const version = "1.0.0"

    return await ofetch(url, {
      headers: { 'User-Agent': `pyramus/${version} (todo@email.com)` },
    })
  } catch (err) {
    if (!isSilent) {
      // TODO: error message
      console.log(`Error fetching ${item}`)
      // handleError({ message: `Error fetching ${item}` })
    }
    console.error(err)
  }
}
