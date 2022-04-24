import express, { json, urlencoded, Request, Response } from "express";
import path from "path";
import axios, { AxiosRequestConfig } from "axios";
import { remove, set } from "lodash";

const node = express();

type load = {
  host: string;
  port: number;
  resp_time: number;
  is_active: boolean;
};

// Application servers
export const servers: load[] = [
  {
    host: "0.0.0.0",
    port: 9000,
    resp_time: 1,
    is_active: true,
  },
];

// Track the last_severed application server to send request
export let last_severed = 0;

node.use(json());
node.use(urlencoded({ extended: true }));

const get_server = (): load | undefined => {
  let server;
  for (const _server of servers) {
    console.log(_server);
    if ((server?.resp_time ?? 0) > _server.resp_time) return;
    server = _server;
  }
  return server;
};

// Forward to application server
export const handler = async (req: Request, res: Response) => {
  try {
    // Destructure following properties from request object
    const { method, url, headers, body } = req as unknown as {
      method: string;
      url: string;
      headers: any;
      body: any;
    };

    // Update track to select next server
    if (last_severed === servers.length - 1) {
      last_severed = 0;
    } else {
      last_severed++;
    }
    let server: load | undefined = get_server();
    if (!server) {
      return res.status(500).send("No running server!");
    }
    console.log(`http://${server.host}:${server.port}${url}`);
    // Requesting to underlying application server
    let startTime = performance.now();
    const response = await axios({
      method,
      url: `http://${server.host}:${server.port}${url}`,
      headers,
      data: body,
    } as AxiosRequestConfig<any>);
    let endTime = performance.now();
    let time_used = endTime - startTime;

    // Send back the response data
    // from application server to client
    set(server, "resp_time", time_used);
    remove(servers, { port: server.port });
    servers.push(server);
    res.send(response.data);
  } catch (err) {
    // Send back the error message
    res.status(500).send("Server error!");
  }
};

// Serve favicon.ico image
node.get("/favicon.ico", (req, res) => res.sendFile("/favicon.ico"));

// When receive new request
// Pass it to handler method
node.use((req, res) => {
  handler(req, res);
});

// Listen on PORT 8080
node.listen(8080, () => {
  console.log("Load Balancer Server " + "listening on PORT 8080");
});
