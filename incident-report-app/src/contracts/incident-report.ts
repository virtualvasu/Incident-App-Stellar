import * as Client from 'incident-report';
import { rpcUrl } from './util';

export default new Client.Client({
  ...Client.networks.testnet,
  rpcUrl,
  allowHttp: true,
});
