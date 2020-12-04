import { Component, OnInit } from "@angular/core";
import { Title } from "@angular/platform-browser";
import { isSameDay, parseISO } from "date-fns";

import { translate } from "src/i18n/translations";

import { Stream } from "src/app/models";
import { ApiService } from "src/app/shared";

@Component({
  selector: "hs-youtube-schedule-stream",
  templateUrl: "./youtube-schedule-stream.component.html",
})
export class YoutubeScheduleStreamComponent implements OnInit {
  constructor(private api: ApiService, private title: Title) {}

  loading = false;
  streamGroup: Array<{ day: Date; streams: Array<Stream> }> = [];
  updatedAt = "";

  ngOnInit() {
    this.title.setTitle(`${translate("youtubeSchedule")} | HoloStats`);

    this.loading = true;
    this.api.getYouTubeScheduleStream().subscribe((res) => {
      this.loading = false;
      this.updatedAt = res.updatedAt;

      let lastStreamSchedule: Date;

      for (const stream of res.streams) {
        const schedule = parseISO(stream.scheduleTime);
        if (lastStreamSchedule && isSameDay(lastStreamSchedule, schedule)) {
          this.streamGroup[this.streamGroup.length - 1].streams.push(stream);
        } else {
          this.streamGroup.push({ day: schedule, streams: [stream] });
        }
        lastStreamSchedule = schedule;
      }
    });
  }

  trackBy(_: number, stream: Stream): string {
    return stream.streamId;
  }
}
