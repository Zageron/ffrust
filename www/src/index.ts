import * as ffrust from "ffrust"
import { Render } from "./render";

Render();

const canvas: HTMLCanvasElement = document.getElementById('canvas') as HTMLCanvasElement;
const context: WebGL2RenderingContext = canvas.getContext("webgl2") as WebGL2RenderingContext;

//ffrust.start_game(canvas, context)
ffrust.webgl2_sample(canvas, context);
