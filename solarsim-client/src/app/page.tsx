import styles from "./page.module.css";
import { brpRequest } from "./requests";
import { displayFetchErrorType } from "./utils/fetch";

export default async function Home() {
  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <div className={styles.intro}>
          <h1>
            {await brpRequest({
              id: 0,
              jsonrpc: "2.0",
              method: "world.get_resources",
              params: { resource: "solarsim_server::HelloWorld" },
            })
              .then((body) => {
                if (body.ok === false) {
                  return displayFetchErrorType(body.error.type);
                } else {
                  return body.data.result.value as string;
                }
              })
              .catch((err) => {
                console.log(err);
                return "Unknown error :(";
              })}
          </h1>
        </div>
      </main>
    </div>
  );
}
