import init, { extract_metadata } from './image_exif_analyzer.js'
import wasmUrl from './image_exif_analyzer_bg.wasm?url'

let loaded

const extractMetadata = async (image) => {
    if(!loaded){
        await init(wasmUrl)
        loaded = true
    }
    const metadata = extract_metadata(new Uint8Array(await image.arrayBuffer()))
    return JSON.parse(metadata).fields
}

export default extractMetadata