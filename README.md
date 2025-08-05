![EN](https://flagcdn.com/w20/gb.png) [English version](https://github.com/AlexLopEx03/image-exif-analyzer/blob/main/README.en.md) of this readme

<div align="center">
  <h1>image-exif-analyzer</h1>
</div>

Librería de NPM para la extracción de metadatos EXIF de imágenes.

***Proyecto de código abierto por AlexLopEx03 bajo licencia MIT*** 📜

---

> [!NOTE]
> Este proyecto envuelve la librería [kamadak-exif](https://crates.io/crates/kamadak-exif) de Cargo (El gestor de paquetes de Rust).
>
> Desarrollado en Rust y transpilado a WebAssembly mediante wasm-pack y wasm-bindgen para poder ser utilizado en el entorno cliente del navegador con un altísimo rendimiento.

<div align="center">
  
| Rust | WebAssembly |
|:----:|:-----------:|
| <img src="https://cdn.simpleicons.org/rust/707070" width="100"/> | <img src="https://upload.wikimedia.org/wikipedia/commons/1/1f/WebAssembly_Logo.svg" width="100"/> |

</div>

---

### Instalación y uso:

```Bash
npm install image-exif-analyzer
```

<br>

```Js
import extractMetadata from 'image-exif-analyzer'

const image = await fetch(image_url)
const metadata = await extractMetadata(image)

console.info(metadata) // -> {
                       //        "DateTime": "2025-01-31 16:30:15",
                       //        "Make": "iphone",
                       //        "Model": "iPhone 13 Pro Max",
                       //        "GPSLatitude": "37/1, 46/1, 5627/100",
                       //         etc ...
                       //    }
```

> [!WARNING]
> Esta librería puede no funcionar en muchos entornos de desarrollo, muchas veces tan solo funcionará tras el empaquetado para producción.
>
> Puedes simular el entorno de producción con los siguientes comandos si utilizas Vite.
>
> ```Bash
> npm run build & npm run preview
> ```

---

> [!TIP]
> 
> - ### Soporte de metadatos EXIF:

> Aunque se soporte EXIF, la mayoría de aplicaciones como redes sociales vacían de metadatos las imágenes por privacidad.
>
> Entre algunas de las aplicaciones que no los eliminan están Drive, Gmail y otros servicios de correos o gestores de archivos.

<div align="center">

| Formato | Extensión | Soporta EXIF |
|:-------:|:---------:|:------------:|
| JPG     | .jpg / .jpeg | ✅ |
| PNG     | .png | ❌ |
| SVG     | .svg | ❌ |
| GIF     | .gif | ❌ |
| AVIF    | .avif | ✅ |
| WEBP    | .webp | ✅ |
| TIFF    | .tif / .tiff | ✅ |
| HEIF    | .heif / .heic | ✅ |
| RAW     | .cr2 / .nef / etc. | ✅ |
| BMP     | .bmp | ❌ |

</div>


<details>
  <summary> Click aquí para ver la lista completa de los metadatos EXIF que se pueden contener:</summary>
  <br>
  <div align="center">
    
  | ⚙️ EXIF |
  | :---------------------------- |
  | ProcessingSoftware            |
  | NewSubfileType                |
  | SubfileType                   |
  | ImageWidth                    |
  | ImageLength (ImageHeight)     |
  | BitsPerSample                 |
  | Compression                   |
  | PhotometricInterpretation     |
  | Thresholding                  |
  | CellWidth                     |
  | CellLength                    |
  | FillOrder                     |
  | DocumentName                  |
  | ImageDescription              |
  | Make                          |
  | Model                         |
  | StripOffsets                  |
  | Orientation                   |
  | SamplesPerPixel               |
  | RowsPerStrip                  |
  | StripByteCounts               |
  | MinSampleValue                |
  | MaxSampleValue                |
  | XResolution                   |
  | YResolution                   |
  | PlanarConfiguration           |
  | PageName                      |
  | XPosition                     |
  | YPosition                     |
  | GrayResponseUnit              |
  | GrayResponseCurve             |
  | T4Options                     |
  | T6Options                     |
  | ResolutionUnit                |
  | PageNumber                    |
  | TransferFunction              |
  | Software                      |
  | DateTime                      |
  | Artist                        |
  | HostComputer                  |
  | Predictor                     |
  | WhitePoint                    |
  | PrimaryChromaticities         |
  | ColorMap                      |
  | HalftoneHints                 |
  | TileWidth                     |
  | TileLength                    |
  | TileOffsets                   |
  | TileByteCounts                |
  | SubIFDs                       |
  | InkSet                        |
  | InkNames                      |
  | NumberOfInks                  |
  | DotRange                      |
  | TargetPrinter                 |
  | ExtraSamples                  |
  | SampleFormat                  |
  | SMinSampleValue               |
  | SMaxSampleValue               |
  | TransferRange                 |
  | ClipPath                      |
  | XClipPathUnits                |
  | YClipPathUnits                |
  | Indexed                       |
  | JPEGTables                    |
  | OPIProxy                      |
  | JPEGProc                      |
  | JPEGInterchangeFormat         |
  | JPEGInterchangeFormatLength   |
  | JPEGRestartInterval           |
  | JPEGLosslessPredictors        |
  | JPEGPointTransforms           |
  | JPEGQTables                   |
  | JPEGDCTables                  |
  | JPEGACTables                  |
  | YCbCrCoefficients             |
  | YCbCrSubSampling              |
  | YCbCrPositioning              |
  | ReferenceBlackWhite           |
  | XMLPacket                     |
  | Rating                        |
  | RatingPercent                 |
  | VignettingCorrParams          |
  | ChromaticAberrationCorrParams |
  | DistortionCorrParams          |
  | ImageID                       |
  | CFARepeatPatternDim           |
  | CFAPattern                    |
  | BatteryLevel                  |
  | Copyright                     |
  | ExposureTime                  |
  | FNumber                       |
  | IPTCNAA                       |
  | ImageResources                |
  | ExifTag                       |
  | InterColorProfile             |
  | ExposureProgram               |
  | SpectralSensitivity           |
  | GPSTag                        |
  | ISOSpeedRatings               |
  | OECF                          |
  | Interlace                     |
  | TimeZoneOffset                |
  | SelfTimerMode                 |
  | DateTimeOriginal              |
  | CompressedBitsPerPixel        |
  | ShutterSpeedValue             |
  | ApertureValue                 |
  | BrightnessValue               |
  | ExposureBiasValue             |
  | MaxApertureValue              |
  | SubjectDistance               |
  | MeteringMode                  |
  | LightSource                   |
  | Flash                         |
  | FocalLength                   |
  | FlashEnergy                   |
  | SpatialFrequencyResponse      |
  | Noise                         |
  | FocalPlaneXResolution         |
  | FocalPlaneYResolution         |
  | FocalPlaneResolutionUnit      |
  | ImageNumber                   |
  | SecurityClassification        |
  | ImageHistory                  |
  | SubjectLocation               |
  | ExposureIndex                 |
  | TIFFEPStandardID              |
  | SensingMethod                 |
  | XPTitle                       |
  | XPComment                     |
  | XPAuthor                      |
  | XPKeywords                    |
  | XPSubject                     |
  | PrintImageMatching            |
  | DNGVersion                    |
  | DNGBackwardVersion            |
  | UniqueCameraModel             |
  | LocalizedCameraModel          |
  | CFAPlaneColor                 |
  | CFALayout                     |
  | LinearizationTable            |
  | BlackLevelRepeatDim           |
  | BlackLevel                    |
  | BlackLevelDeltaH              |
  | BlackLevelDeltaV              |
  | WhiteLevel                    |
  | DefaultScale                  |
  | DefaultCropOrigin             |
  | DefaultCropSize               |
  | ColorMatrix1                  |
  | ColorMatrix2                  |
  | CameraCalibration1            |
  | CameraCalibration2            |
  | ReductionMatrix1              |
  | ReductionMatrix2              |
  | AnalogBalance                 |
  | AsShotNeutral                 |
  | AsShotWhiteXY                 |
  | BaselineExposure              |
  | BaselineNoise                 |
  | BaselineSharpness             |
  | BayerGreenSplit               |
  | LinearResponseLimit           |
  | CameraSerialNumber            |
  | LensInfo                      |
  | ChromaBlurRadius              |
  | AntiAliasStrength             |
  | ShadowScale                   |
  | DNGPrivateData                |
  | MakerNoteSafety               |
  | CalibrationIlluminant1        |
  | CalibrationIlluminant2        |
  | BestQualityScale              |
  | RawDataUniqueID               |
  | OriginalRawFileName           |
  | OriginalRawFileData           |
  | ActiveArea                    |
  | MaskedAreas                   |
  | AsShotICCProfile              |
  | AsShotPreProfileMatrix        |
  | CurrentICCProfile             |
  | CurrentPreProfileMatrix       |
  | ColorimetricReference         |
  | ExifVersion                   |
  | DateTimeOriginal              |
  | DateTimeDigitized             |
  | OffsetTime                    |
  | OffsetTimeOriginal            |
  | OffsetTimeDigitized           |
  | ComponentsConfiguration       |
  | MakerNote                     |
  | UserComment                   |
  | SubSecTime                    |
  | SubSecTimeOriginal            |
  | SubSecTimeDigitized           |
  | Temperature                   |
  | Humidity                      |
  | Pressure                      |
  | WaterDepth                    |
  | Acceleration                  |
  | CameraElevationAngle          |
  | FlashpixVersion               |
  | ColorSpace                    |
  | PixelXDimension               |
  | PixelYDimension               |
  | RelatedSoundFile              |
  | InteroperabilityTag           |
  | GPSVersionID                  |
  | GPSLatitudeRef                |
  | GPSLatitude                   |
  | GPSLongitudeRef               |
  | GPSLongitude                  |
  | GPSAltitudeRef                |
  | GPSAltitude                   |
  | GPSTimeStamp                  |
  | GPSSatellites                 |
  | GPSStatus                     |
  | GPSMeasureMode                |
  | GPSDOP                        |
  | GPSSpeedRef                   |
  | GPSSpeed                      |
  | GPSTrackRef                   |
  | GPSTrack                      |
  | GPSImgDirectionRef            |
  | GPSImgDirection               |
  | GPSMapDatum                   |
  | GPSDestLatitudeRef            |
  | GPSDestLatitude               |
  | GPSDestLongitudeRef           |
  | GPSDestLongitude              |
  | GPSDestBearingRef             |
  | GPSDestBearing                |
  | GPSDestDistanceRef            |
  | GPSDestDistance               |
  | GPSProcessingMethod           |
  | GPSAreaInformation            |
  | GPSDateStamp                  |
  | GPSDifferential               |
  | GPSHPositioningError          |
  | InteroperabilityIndex         |
  | InteroperabilityVersion       |
  | RelatedImageFileFormat        |
  | RelatedImageWidth             |
  | RelatedImageLength            |

  <br>

  [Volver arriba](#soporte-de-metadatos-exif)
  
  </div>
</details>

<br>

####  Para cualquier duda o comentario acerca del proyecto, puedes dirigirte a la sección de Discussions.