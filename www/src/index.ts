import * as ffrust from "ffrust"
import { Render } from "./render";

Render();

const canvas: HTMLCanvasElement = document.getElementById('canvas') as HTMLCanvasElement;
const context: WebGL2RenderingContext = canvas.getContext("webgl2") as WebGL2RenderingContext;
//const context: CanvasRenderingContext2D = canvas.getContext("2d") as CanvasRenderingContext2D;

//ffrust.start_game(canvas, context)
ffrust.webgl2_sample(canvas, context);
