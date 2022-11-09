FROM node:18.7.0
WORKDIR /app

COPY package-lock.json package-lock.json
COPY package.json package.json

RUN npm install -g @ionic/cli native-run cordova-res
RUN npm ci

COPY . .

CMD ionic serve --port 3001 --host 0.0.0.0
