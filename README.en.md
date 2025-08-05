![ES](https://flagcdn.com/w20/es.png) [Versión en Español](https://github.com/AlexLopEx03/image-exif-analyzer/blob/main/README.md) de este readme

<div align="center">
  <h1>image-exif-analyzer</h1>
</div>

NPM library for extracting image EXIF metadata.

***Open source project by AlexLopEx03 under the MIT license*** 📜

---

> [!NOTE]
> This project is a wrapper for the Cargo library [kamadak-exif](https://crates.io/crates/kamadak-exif).
>
> Developed in Rust and prepared to WebAssembly using wasm-pack and wasm-bindgen, so it can be used in the browser client-side with very high performance.

<div align="center">
  
| Rust | WebAssembly |
|:----:|:-----------:|
| <img src="https://cdn.simpleicons.org/rust/707070" width="100"/> | <img src="https://upload.wikimedia.org/wikipedia/commons/1/1f/WebAssembly_Logo.svg" width="100"/> |

</div>

---

### Installation and usage:

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
> This library may not work in many development environments, sometimes it will only work in production environments, after being bundled.
>
> You can simulate the production environment with the following commands if you use Vite
>
> ```Bash
> npm run build & npm run preview
> ```

---

> [!TIP]
> 
> - ### Exif metadata support:

> Although EXIF is supported, most applications such as social networks strip metadata from images for privacy reasons.
>
> Some applications that **do not** remove them include Drive, Gmail, and other email services or file managers.

<div align="center">

| Format | Extension | Supports EXIF |
|:------:|:---------:|:-------------:|
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
  <summary> Click here to see the full list of possible EXIF metadata:</summary>
  <br>
  <div align="center">
    
  | ⚙️ Exif |
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

  [Back to top](#exif-metadata-support)
  
  </div>
</details>

<br>

####  If you have any questions or comments about the project, you can direct them to the Discussions section.