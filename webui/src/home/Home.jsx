import { useState, useParams, useEffect } from 'react'
import './Home.css'
import Request from '../Request.jsx'
let bonusItems = [];

function Bonus() {
    const [inputValue, setInputValue] = useState('');
    const error = useState("");
    
    let itemz = [];
    bonusItems.forEach((e) => {
        itemz.push(e.master_login_bonus_id);
    })
    
    const [submittedItems, setSubmittedItems] = useState(itemz);
    
    const handleSubmit = async (event) => {
        event.preventDefault();
        let input = parseInt(inputValue.trim());
        if (isNaN(input) || submittedItems.includes(input)) return;
        let resp = await Request("/api/webui/startLoginbonus", {
            bonus_id: input
        });
        if (resp.result !== "OK") {
            error[1](resp.message);
            return;
        }
    
        setSubmittedItems([...submittedItems, resp.id]);
        setInputValue('');
    };

    const handleRemoveItem = (index) => {
        const updatedItems = [...submittedItems];
        updatedItems.splice(index, 1);
        setSubmittedItems(updatedItems);
    };
//                <button onClick={() => handleRemoveItem(index)}>X</button>
    return (
        <div>
          <h2>Current login bonus list</h2>
          <div id="error"> { error[0] ? <p>Error: { error[0] } </p> : <p></p> } </div>
          <ul>
            {submittedItems.map((item, index) => (
              <li key={index}>
                {item}
              </li>
            ))}
          </ul>
          <form onSubmit={handleSubmit}>
            <input
              type="text"
              value={inputValue}
              onChange={(event) => setInputValue(event.target.value)}
              placeholder="Enter login bonus ID"
            />
            <button type="submit">Submit</button>
          </form>
          <p>You can find a list of available login bonus IDs <a href="https://github.com/ethanaobrien/ew/blob/main/src/router/json/login_bonus.json">here</a>. You should input the <code>id</code> field</p>
        </div>
    );
}

function Home() {
    const [user, userdata] = useState();
    
    const logout = () => {
        window.location.href = "/webui/logout";
    }
    
    useEffect(() => {
        if (user) return;
        (async () => {
            let resp = await Request("/api/webui/userInfo");
            if (resp.result !== "OK") {
                window.location.href = "/?message=" + encodeURIComponent(resp.message);
                return;
            }
            let user = resp.data.userdata;
            bonusItems = resp.data.loginbonus.bonus_list;
            /*
            bonusItems = [{"master_login_bonus_id":1,"day_counts":[1,2],"event_bonus_list":[]}];
            let user = {
                user: {
                    id: 1,
                    rank: 3,
                    exp: 10,
                    last_login_time: 5
                }
            }*/
            userdata(
                <div>
                    <p>User id: { user.user.id } </p>
                    <p>Rank: { user.user.rank } ({ user.user.exp } exp)</p>
                    <p>Last Login: { (new Date(user.user.last_login_time * 1000)).toString() } </p>
                    <Bonus />
                </div>
            );
        })();
    });
  
    return (
        <div id="home">
            <button id="logout" onClick={logout}>Logout</button>
            <h1>Home</h1>
            
            { user ? <div> { user } </div> : <p>Loading...</p> }
        </div>
    );
}

export default Home;