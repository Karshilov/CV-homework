import { wasm_rgb2grey } from '../../src/lib.rs';
(async function main () {
    let app = document.getElementById("app");
    const img = new Image();
    const url = "https://upload-bbs.mihoyo.com/upload/2021/08/10/283837710/89f72b8ab1ba23efca0391aa5645f7ac_7056506055197632319.jpg?x-oss-process=image/resize,s_600/quality,q_80/auto-orient,0/interlace,1/format,jpg";
    img.src = url;
    img.crossOrigin = "Anonymous";
    await new Promise((resolve, reject) => { setTimeout(resolve, 1000)});
    const cvs = document.createElement("canvas");
    const canvas = cvs.getContext("2d");
    canvas.drawImage(img, 0, 0);
    const data = canvas.getImageData(0, 0, img.width, img.height).data;
    let greyData = [];
    wasm_rgb2grey(data, greyData);
    const newImageData = ImageData(greyData, img.width, img.height);
    canvas.putImageData(newImageData, 0, 0);
    app.appendChild(cvs);
})()