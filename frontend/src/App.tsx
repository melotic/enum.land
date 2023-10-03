import { useDebounce } from "@uidotdev/usehooks";
import axios from "axios";
import { useState } from "react";
import {
  Alert,
  Container,
  Form,
  InputGroup,
  ListGroup,
  Navbar,
  Spinner,
} from "react-bootstrap";
import { TbNetwork } from "react-icons/tb";
import { useQuery } from "react-query";

function App() {
  const [domain, setDomain] = useState<string>("");
  const debouncedInput = useDebounce(domain, 500);
  const { isLoading, error, data } = useQuery<string[], Error>(
    ["domains", debouncedInput],
    async () => {
      const data = await axios.get(`/api/domain?q=${debouncedInput}`);
      return data.data.sort();
    },
    { enabled: debouncedInput.length > 0 },
  );

  return (
    <>
      <Navbar className="bg-body-tertiary">
        <Container>
          <Navbar.Brand href="#home">
            <TbNetwork size={32} />
            enum.land
          </Navbar.Brand>
        </Container>
      </Navbar>
      <Container className="mt-3">
        <InputGroup>
          <InputGroup.Text>domain</InputGroup.Text>
          <Form.Control
            placeholder="example.com"
            value={domain}
            onChange={(e) => setDomain(e.target.value)}
          />
        </InputGroup>
        <Container className="mt-3">
          {isLoading && <Spinner />}
          {error && <Alert variant="danger">{error.message}</Alert>}
          {data && (
            <div>
              <h3>Subdomains</h3>
              <ListGroup>
                {data.map((subdomain) => (
                  <ListGroup.Item>{subdomain}</ListGroup.Item>
                ))}
              </ListGroup>
            </div>
          )}
        </Container>
      </Container>
    </>
  );
}

export default App;
